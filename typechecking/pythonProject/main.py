from ortools.sat.python import cp_model


def solve(instance):
    model = cp_model.CpModel()

    teams = instance.teams

    U_max = instance.m


    T_max = instance.n
    T = range(T_max)

    F = [model.NewIntVar(0, U - 1, f'y{s}') for s in T]

    for num, constraint in enumerate(instance.constraints):
        match (num, constraint.__class__.__name__):



            case _, "AuthorisationConstraint":
                for s in T:
                    if s not in constraint.tasks:
                        model.Add(F[s] != constraint.user)

            case _, "SeparationOfDutyConstraint":
                model.Add(F[constraint.t1] != F[constraint.t2])

            case _, "BindingOfDutyConstraint":
                model.Add(F[constraint.t1] == F[constraint.t2])

            case _, "AtMostKConstraint":
                # Create a list to track unique user assignments
                user_assigned = [model.NewBoolVar(f"user_assigned_{u}") for u in range(users)]

                # Create constraints to track if a user is assigned to any task in the scope
                for u in range(users):
                    # Create a list of constraints for the user being assigned to tasks
                    tasks_assigned_to_user = [model.NewBoolVar(f"task_{tsk}_assigned_to_{u}") for tsk in constraint.tasks]

                    # Add equality constraints for each task assignment
                    for i, tsk in enumerate(constraint.tasks):
                        model.Add(F[tsk] == u).OnlyEnforceIf(tasks_assigned_to_user[i])
                        model.Add(F[tsk] != u).OnlyEnforceIf(tasks_assigned_to_user[i].Not())

                    # Add a max-equality constraint: user_assigned[u] will be True if any task is assigned to this user
                    model.AddMaxEquality(user_assigned[u], tasks_assigned_to_user)

                # Count how many unique users are assigned to the tasks in the scope
                model.Add(sum(user_assigned) <= constraint.k)

            case _, "ExtensionConstraint1":
                # Create a list to track unique user assignments
                user_assigned = [model.NewBoolVar(f"user_assigned_{u}") for u in range(users)]

                # Create constraints to track if a user is assigned to any task in the scope
                for u in range(users):
                    # Create a list of constraints for the user being assigned to tasks
                    tasks_assigned_to_user = [model.NewBoolVar(f"task_{tsk}_assigned_to_{u}") for tsk in constraint.tasks]

                    # Add equality constraints for each task assignment
                    for i, tsk in enumerate(constraint.tasks):
                        model.Add(F[tsk] == u).OnlyEnforceIf(tasks_assigned_to_user[i])
                        model.Add(F[tsk] != u).OnlyEnforceIf(tasks_assigned_to_user[i].Not())

                    # Add a max-equality constraint: user_assigned[u] will be True if any task is assigned to this user
                    model.AddMaxEquality(user_assigned[u], tasks_assigned_to_user)

                # Count how many unique users are assigned to the tasks in the scope
                model.Add(sum(user_assigned) == constraint.k)

            case _, "ExtensionConstraint2":
                team_indexes = constraint.teams
                chosen_teams = []
                for team_index in team_indexes:
                    team_chosen = model.NewBoolVar('')
                    chosen_teams.append(team_chosen)
                    num_steps_assigned = [model.NewBoolVar(f"step_{x}") for x in range(len(constraint.tasks))]
                    team = teams[team_index]
                    for step in range(len(constraint.tasks)):
                        assigned_to_team = [model.NewBoolVar(f'assigned_{x}') for x in range(len(team))]
                        for x in range(len(team)):
                            model.Add(F[constraint.tasks[step]] == team[x]).OnlyEnforceIf(assigned_to_team[x])
                            model.Add(F[constraint.tasks[step]] != team[x]).OnlyEnforceIf(assigned_to_team[x].Not())
                        model.AddMaxEquality(num_steps_assigned[step], assigned_to_team)
                    model.Add(sum(num_steps_assigned) == len(constraint.tasks)).OnlyEnforceIf(team_chosen)
                    model.Add(sum(num_steps_assigned) == 0).OnlyEnforceIf(team_chosen.Not())
                model.Add(sum(chosen_teams) == 1)

            case _, "ExtensionConstraint3":
                team_index = constraint.team
                team = teams[team_index]
                k = constraint.k

                task_assigned_to_group = [
                    [model.NewBoolVar(f'task_{step}_assigned_to_{user}') for user in team]
                    for step in T
                ]

                # Make values for each step to each user in the current team
                for step in T:
                    for user_index, user in enumerate(team):
                        model.Add(F[step] == user).OnlyEnforceIf(task_assigned_to_group[step][user_index])
                        model.Add(F[step] != user).OnlyEnforceIf(task_assigned_to_group[step][user_index].Not())

                # Number of tasks assigned to the group should be less than, or equal to K
                def add_task_assignment(arr, idx):
                    a = model.NewIntVar(0, T_max, f"intvar_arr_{idx}")
                    model.Add(sum(arr) == a)
                    return a

                group_task_assignments = [add_task_assignment(arr, idx) for idx, arr in enumerate(task_assigned_to_group)]
                group_tasks = model.NewIntVar(0, T_max, "group_tasks")
                model.Add(k >= sum(group_task_assignments))

            case _, "ExtensionConstraint4":
                team_index = constraint.team
                team = teams[team_index]
                supervisor = constraint.supervisor

                task_assigned_to_group = [
                    [model.NewBoolVar(f'task_{step}_assigned_to_{user}') for user in team]
                    for step in T
                ]

                # Get all of the tasks assigned to the group.
                for step in T:
                    for user_index, user in enumerate(team):
                        model.Add(F[step] == user).OnlyEnforceIf(task_assigned_to_group[step][user_index])
                        model.Add(F[step] != user).OnlyEnforceIf(task_assigned_to_group[step][user_index].Not())

                def add_task_assignment(arr, idx):
                    a = model.NewIntVar(0, T_max, f"intvar_arr_{idx}")
                    model.Add(sum(arr) == a)
                    return a

                group_task_assignments = [add_task_assignment(arr, idx) for idx, arr in enumerate(task_assigned_to_group)]
                group_tasks = model.NewIntVar(0, T_max, "group_tasks")
                model.Add(group_tasks == sum(group_task_assignments))

                # Get all of the tasks assigned to the supervisor.
                supervising_tasks = [model.NewBoolVar(f'task_{step}_assigned_to_supervisor_{supervisor}') for step in T]
                for step in T:
                    model.Add(F[step] == supervisor).OnlyEnforceIf(supervising_tasks[step])
                    model.Add(F[step] != supervisor).OnlyEnforceIf(supervising_tasks[step].Not())
                supervisor_tasks = model.NewIntVar(0, T_max, "supervisor_tasks_int")
                model.Add(supervisor_tasks == sum(supervising_tasks))

                # Create Boolean conditions
                group_tasks_bool = model.NewBoolVar('group_tasks_bool')
                supervisor_tasks_bool = model.NewBoolVar('supervisor_tasks_bool')
                model.Add(group_tasks >= 1).OnlyEnforceIf(group_tasks_bool)
                model.Add(group_tasks < 1).OnlyEnforceIf(group_tasks_bool.Not())
                model.Add(supervisor_tasks >= 1).OnlyEnforceIf(supervisor_tasks_bool)
                model.Add(supervisor_tasks < 1).OnlyEnforceIf(supervisor_tasks_bool.Not())

                # If a task is assigned in the group, the supervisor must be assigned a task.
                model.AddImplication(group_tasks_bool, supervisor_tasks_bool)

    solver = cp_model.CpSolver()
    status = solver.Solve(model)

    if status in [cp_model.OPTIMAL, cp_model.FEASIBLE]:
        solution = Solution(instance, True)
        for x in T:
            solution.assign_user(x, solver.Value(F[x]))
        return solution

    else:
        solution = Solution(instance, False)
        return solution
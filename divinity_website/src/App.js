import React, { useState } from 'react';
import { Controlled as CodeMirror } from 'react-codemirror2';
import 'codemirror/lib/codemirror.css'; 
import 'codemirror/theme/dracula.css'; 
import 'codemirror/mode/javascript/javascript';

function App() {
  const [code, setCode] = useState('// Write your code here');

  const handleChange = (editor, data, value) => {
    setCode(value);
  };

  return (
    <div className="App">
      <h1>CodeMirror in React</h1>
      <CodeMirror
        value={code}
        options={{
          mode: 'javascript',
          theme: 'dracula',
          lineNumbers: true,
          lineWrapping: true,
        }}
        onBeforeChange={handleChange}
      />
    </div>
  );
}

export default App;

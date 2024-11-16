import React, { useState } from 'react';
import { Controlled as CodeMirror } from 'react-codemirror2';
import 'codemirror/lib/codemirror.css';  // You can import the styles here
import 'codemirror/theme/dracula.css';  // Import the theme you want

// You can import additional modes for different languages
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

const theme = {
  palette: {
    primary: '#3500d3',
    secondary: '#0c0032',
    attention: '#240090',
    background: {
      body: '#282828',
      base: '#240090',
    },
    text: {
      primary: '#FFF',
    },
  },
  spacing: (multiplier = 1) => `${4 * multiplier}px`,
  borderRadius: '4px',
  typography: {
    h4: {
      'font-weight': 'bold',
      'font-size': '28px',
    },
    h5: {
      'font-weight': 'bold',
      'font-size': '22px',
    },
    h6: {
      'font-weight': 'bold',
      'font-size': '16px',
    },
    body: {
      'font-weight': 'normal',
      'font-size': '14px',
    },
  },
};

export default theme;
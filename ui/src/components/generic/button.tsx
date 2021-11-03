import styled from "styled-components";
import theme from "../../theme";

const Button = styled.button`
  background: transparent;
  border-radius: ${theme.borderRadius};
  padding: ${theme.spacing(1)};
  border: none;
  background: ${theme.palette.primary};
  text-transform: uppercase;
  ${theme.typography.h6}
  &:hover {
    background: ${theme.palette.primary};
  }
  &:active {
    background: ${theme.palette.attention};
  }
`;


export default Button;
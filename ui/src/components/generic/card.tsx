import styled from "styled-components";
import theme from "../../theme";

const Card = styled.div`
  background-color: ${theme.palette.background.base};
  border-radius: ${theme.borderRadius};
  padding: ${theme.spacing(3)};
  margin: ${theme.spacing(2)};
`;

export default Card;

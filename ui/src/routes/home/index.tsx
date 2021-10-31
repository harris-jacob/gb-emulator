import { FunctionalComponent, h } from 'preact';
import style from './style.css';
import Emulator from '../../emulator/Emulator';

const Home: FunctionalComponent = () => {
    return (
        <div class={style.home}>
            <h1>Home</h1>
            <Emulator/>
            <p>This is the Home component.</p>
        </div>
    );
};

export default Home;

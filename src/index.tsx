import {
  ButtonItem,
  definePlugin,
  DialogButton,
  Menu,
  MenuItem,
  PanelSection,
  PanelSectionRow,
  Router,
  ServerAPI,
  showContextMenu,
  staticClasses,
} from "decky-frontend-lib";
import { VFC } from "react";
import { FaLightbulb } from "react-icons/fa";

interface SetColorCommandArgs {
  color: [number, number, number]
};


const Content: VFC<{ serverAPI: ServerAPI }> = ({serverAPI}) => {
  // const [result, setResult] = useState<number | undefined>();

  // const onClick = async () => {
  //   const result = await serverAPI.callPluginMethod<AddMethodArgs, number>(
  //     "add",
  //     {
  //       left: 2,
  //       right: 2,
  //     }
  //   );
  //   if (result.success) {
  //     setResult(result.result);
  //   }
  // };

  const setColor = async(color: [number, number, number]) =>  {
    await serverAPI.callPluginMethod<SetColorCommandArgs, string>("set_color", { color });
  }

  return (
    <PanelSection title="Panel Section">
      <PanelSectionRow>
        <ButtonItem onClick={() => setColor([255, 0, 0])}>Set Red</ButtonItem>
        <ButtonItem onClick={() => setColor([0, 255, 0])}>Set Green</ButtonItem>
        <ButtonItem onClick={() => setColor([255, 255, 0])}>Set Yello</ButtonItem>
      </PanelSectionRow>
    </PanelSection>
  );
};

const DeckyPluginRouterTest: VFC = () => {
  return (
    <div style={{ marginTop: "50px", color: "white" }}>
      Hello World!
      <DialogButton onClick={() => Router.NavigateToLibraryTab()}>
        Go to Library
      </DialogButton>
    </div>
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute("/decky-plugin-test", DeckyPluginRouterTest, {
    exact: true,
  });

  return {
    title: <div className={staticClasses.Title}>LED Control</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <FaLightbulb />,
    onDismount() {
      serverApi.routerHook.removeRoute("/decky-plugin-test");
    },
  };
});

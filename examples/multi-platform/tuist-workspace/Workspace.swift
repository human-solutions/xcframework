import ProjectDescription

let workspace = Workspace(
    name: "CargoXCFramework",
    projects: ["App-iOS", "App-macOS", "Shared"],
    generationOptions: .options(autogeneratedWorkspaceSchemes: .disabled)
)

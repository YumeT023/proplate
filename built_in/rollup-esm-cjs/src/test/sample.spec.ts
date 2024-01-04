import {getProjectName} from "..";

describe("Project", () => {
  test("It should return project name", () => {
    expect(getProjectName()).toEqual("$project_name");
  });
});

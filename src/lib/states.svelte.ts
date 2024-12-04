import type { Course } from "./course";

export interface SelectableCourse extends Course {
  selected: boolean;
}

export const userState: { courses: SelectableCourse[]; voting: boolean } =
  $state({
    courses: [],
    voting: false,
  });

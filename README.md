# abstract_form
abstract_form is a set of traits and structs modelling a form for user input, detached from the actual interface representation. Forms retain the structure typical of paper forms, being made up of control groups, with control groups containing controls. Controls, however, describe only the value type and type-specific constraints.

Control names are strings. Control values can be:
 - string
 - integer
 - float

Further, controls values can be:
 - Freeform
 - Closed choice

Forms and controls are meant to be rendered, filled and validated. Actual implementation of the user facing interface is left outside the scope of this crate. This crate is a companion of abstract_form_derive, which implements form derivation from rust structs.

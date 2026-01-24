/*****************************************
 * UI/Style Definitions, Component architecture
 *****************************************/

/****************************************************
 * 'Bits' are the highest level UI component
 *      'Bits' are comprised of 'Pieces',
 *      which are the lowest level UI Components
 *
 *      For now, styles can only be applied to Pieces
 ****************************************************/

/*
UPDATE:

'bits' will be renamed to App (singular), and
'pieces' will be renamed to Components.

See bits/mod.rs for more info.
*/
pub mod bits;

// Style definitions
pub mod styles;

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113274726: FileFormat = FileFormat {
    id: 113_274_726,
    puid: "wikidata/113274726",
    name: "The Print Shop Deluxe Label",
    extensions: &["pda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

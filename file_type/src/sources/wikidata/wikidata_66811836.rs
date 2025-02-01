use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66811836: FileFormat = FileFormat {
    id: 66_811_836,
    puid: "wikidata/66811836",
    name: "Inform source code file",
    extensions: &["inf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

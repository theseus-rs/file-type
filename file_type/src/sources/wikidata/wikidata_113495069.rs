use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113495069: FileFormat = FileFormat {
    id: 113_495_069,
    puid: "wikidata/113495069",
    name: "Calc602 Macro File",
    extensions: &["mc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

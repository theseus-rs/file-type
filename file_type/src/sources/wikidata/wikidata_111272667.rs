use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272667: FileFormat = FileFormat {
    id: 111_272_667,
    puid: "wikidata/111272667",
    name: "Logic EXS24 instrument file",
    extensions: &["exs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

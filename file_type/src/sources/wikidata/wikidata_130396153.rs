use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130396153: FileFormat = FileFormat {
    id: 130_396_153,
    source_type: SourceType::Wikidata,
    name: "Ooc source code file",
    extensions: &["ooc"],
    media_types: &["text/x-ooc"],
    signatures: &[],
    related_formats: &[],
};

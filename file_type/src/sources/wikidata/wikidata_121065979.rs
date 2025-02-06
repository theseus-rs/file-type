use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121065979: FileFormat = FileFormat {
    id: 121_065_979,
    source_type: SourceType::Wikidata,
    name: "Wizard Database",
    extensions: &["mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

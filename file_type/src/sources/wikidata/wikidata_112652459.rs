use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652459: FileFormat = FileFormat {
    id: 112_652_459,
    source_type: SourceType::Wikidata,
    name: "Astound file format",
    extensions: &["asd"],
    media_types: &["x-application/Astound"],
    signatures: &[],
    related_formats: &[],
};

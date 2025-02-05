use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355515: FileFormat = FileFormat {
    id: 111_355_515,
    source_type: SourceType::Wikidata,
    name: "Talking Technology Incorporated file",
    extensions: &["vox"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

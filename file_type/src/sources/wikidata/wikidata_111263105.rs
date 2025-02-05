use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263105: FileFormat = FileFormat {
    id: 111_263_105,
    source_type: SourceType::Wikidata,
    name: "Typhoon wave file",
    extensions: &["c01"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

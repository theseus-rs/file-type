use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009215: FileFormat = FileFormat {
    id: 111_009_215,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Banner File Format",
    extensions: &["ban"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

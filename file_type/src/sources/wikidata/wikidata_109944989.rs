use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109944989: FileFormat = FileFormat {
    id: 109_944_989,
    source_type: SourceType::Wikidata,
    name: "Ulead Template File",
    extensions: &["tpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

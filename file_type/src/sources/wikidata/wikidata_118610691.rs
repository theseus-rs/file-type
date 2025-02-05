use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118610691: FileFormat = FileFormat {
    id: 118_610_691,
    source_type: SourceType::Wikidata,
    name: "Visual C++ IntelliSense Database file",
    extensions: &["ncb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

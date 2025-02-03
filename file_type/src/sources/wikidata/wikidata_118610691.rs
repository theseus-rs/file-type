use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118610691: FileFormat = FileFormat {
    id: 118_610_691,
    source_type: SourceType::Wikidata,
    name: "Visual C++ IntelliSense Database file",
    extensions: &["ncb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

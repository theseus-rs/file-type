use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117886050: FileFormat = FileFormat {
    id: 117_886_050,
    source_type: SourceType::Wikidata,
    name: "BZip3",
    extensions: &["bz3"],
    media_types: &["application/x-bzip3"],
    internal_signatures: &[],
    related_formats: &[],
};

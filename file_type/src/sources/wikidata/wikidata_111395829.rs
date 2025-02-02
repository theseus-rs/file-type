use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111395829: FileFormat = FileFormat {
    id: 111_395_829,
    source_type: SourceType::Wikidata,
    name: "PhotoSuite Project File",
    extensions: &["pzp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

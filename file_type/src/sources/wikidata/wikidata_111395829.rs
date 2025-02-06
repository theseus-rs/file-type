use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111395829: FileFormat = FileFormat {
    id: 111_395_829,
    source_type: SourceType::Wikidata,
    name: "PhotoSuite Project File",
    extensions: &["pzp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

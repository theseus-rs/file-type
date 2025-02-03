use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116376076: FileFormat = FileFormat {
    id: 116_376_076,
    source_type: SourceType::Wikidata,
    name: "Access Database Addins",
    extensions: &["mda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

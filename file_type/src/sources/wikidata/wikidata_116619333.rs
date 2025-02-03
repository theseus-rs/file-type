use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116619333: FileFormat = FileFormat {
    id: 116_619_333,
    source_type: SourceType::Wikidata,
    name: "Amiga SVX",
    extensions: &["svx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

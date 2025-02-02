use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967349: FileFormat = FileFormat {
    id: 27_967_349,
    source_type: SourceType::Wikidata,
    name: "iTunes Music Library, binary variant",
    extensions: &["itl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73513552: FileFormat = FileFormat {
    id: 73_513_552,
    source_type: SourceType::Wikidata,
    name: "Puppy Linux DotPup installer package",
    extensions: &["pup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

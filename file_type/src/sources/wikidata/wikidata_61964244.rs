use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61964244: FileFormat = FileFormat {
    id: 61_964_244,
    source_type: SourceType::Wikidata,
    name: "pulse EKKO header file",
    extensions: &["hd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

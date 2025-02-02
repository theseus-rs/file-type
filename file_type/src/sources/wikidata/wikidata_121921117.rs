use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121921117: FileFormat = FileFormat {
    id: 121_921_117,
    source_type: SourceType::Wikidata,
    name: "Ptex File Format",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

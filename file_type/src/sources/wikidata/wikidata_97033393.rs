use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_97033393: FileFormat = FileFormat {
    id: 97_033_393,
    source_type: SourceType::Wikidata,
    name: "Magick Scripting Language",
    extensions: &["msl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

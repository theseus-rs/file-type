use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97033393: FileFormat = FileFormat {
    id: 97_033_393,
    source_type: SourceType::Wikidata,
    name: "Magick Scripting Language",
    extensions: &["msl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

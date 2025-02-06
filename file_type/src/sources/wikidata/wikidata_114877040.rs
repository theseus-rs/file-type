use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877040: FileFormat = FileFormat {
    id: 114_877_040,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money Backup File",
    extensions: &["mbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771322: FileFormat = FileFormat {
    id: 28_771_322,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database file format (backup file)",
    extensions: &["bdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

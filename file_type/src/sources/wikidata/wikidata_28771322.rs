use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771322: FileFormat = FileFormat {
    id: 28_771_322,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database file format (backup file)",
    extensions: &["bdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

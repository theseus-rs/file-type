use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_45028191: FileFormat = FileFormat {
    id: 45_028_191,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Backup",
    extensions: &["xlk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

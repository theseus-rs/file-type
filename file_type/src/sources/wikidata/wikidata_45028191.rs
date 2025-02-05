use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45028191: FileFormat = FileFormat {
    id: 45_028_191,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Backup",
    extensions: &["xlk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

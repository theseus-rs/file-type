use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97037799: FileFormat = FileFormat {
    id: 97_037_799,
    source_type: SourceType::Wikidata,
    name: "HP Page Control Language",
    extensions: &["pcl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

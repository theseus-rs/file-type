use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_97037799: FileFormat = FileFormat {
    id: 97_037_799,
    source_type: SourceType::Wikidata,
    name: "HP Page Control Language",
    extensions: &["pcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131850479: FileFormat = FileFormat {
    id: 131_850_479,
    source_type: SourceType::Wikidata,
    name: "OpenVDB file format",
    extensions: &["vdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

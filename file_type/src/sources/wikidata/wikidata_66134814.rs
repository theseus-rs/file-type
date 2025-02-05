use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66134814: FileFormat = FileFormat {
    id: 66_134_814,
    source_type: SourceType::Wikidata,
    name: "Access Add-in file format",
    extensions: &["mda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

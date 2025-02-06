use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855798: FileFormat = FileFormat {
    id: 105_855_798,
    source_type: SourceType::Wikidata,
    name: "Delphi Options File",
    extensions: &["dof"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48911845: FileFormat = FileFormat {
    id: 48_911_845,
    source_type: SourceType::Wikidata,
    name: "Hewlett Packard AdvanceWrite Text File",
    extensions: &["aw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

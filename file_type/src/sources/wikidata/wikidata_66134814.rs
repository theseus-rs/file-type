use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66134814: FileFormat = FileFormat {
    id: 66_134_814,
    source_type: SourceType::Wikidata,
    name: "Access Add-in file format",
    extensions: &["mda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

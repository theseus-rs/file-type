use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66146236: FileFormat = FileFormat {
    id: 66_146_236,
    source_type: SourceType::Wikidata,
    name: "InfoPath Form Definition File",
    extensions: &["xsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

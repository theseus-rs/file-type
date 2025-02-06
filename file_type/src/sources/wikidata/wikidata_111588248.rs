use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111588248: FileFormat = FileFormat {
    id: 111_588_248,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Interchange Document",
    extensions: &["inx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

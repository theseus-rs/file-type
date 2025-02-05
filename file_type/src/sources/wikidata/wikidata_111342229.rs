use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342229: FileFormat = FileFormat {
    id: 111_342_229,
    source_type: SourceType::Wikidata,
    name: "Sounder sound file",
    extensions: &["sndr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

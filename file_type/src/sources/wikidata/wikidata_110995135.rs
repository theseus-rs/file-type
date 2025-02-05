use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110995135: FileFormat = FileFormat {
    id: 110_995_135,
    source_type: SourceType::Wikidata,
    name: "Asymetrix 3D Scene",
    extensions: &["scn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

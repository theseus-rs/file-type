use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4928413: FileFormat = FileFormat {
    id: 4_928_413,
    source_type: SourceType::Wikidata,
    name: "Blorb",
    extensions: &["blb", "blorb", "gblorb", "glb", "zblorb", "zlb"],
    media_types: &["application/x-blorb"],
    signatures: &[],
    related_formats: &[],
};

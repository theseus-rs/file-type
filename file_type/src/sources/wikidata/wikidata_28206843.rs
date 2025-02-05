use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206843: FileFormat = FileFormat {
    id: 28_206_843,
    source_type: SourceType::Wikidata,
    name: "Palm Database ImageViewer",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

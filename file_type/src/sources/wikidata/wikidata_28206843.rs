use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206843: FileFormat = FileFormat {
    id: 28_206_843,
    source_type: SourceType::Wikidata,
    name: "Palm Database ImageViewer",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

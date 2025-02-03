use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61913269: FileFormat = FileFormat {
    id: 61_913_269,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project Export File, version 4",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    internal_signatures: &[],
    related_formats: &[],
};

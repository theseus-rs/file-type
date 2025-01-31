use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61913269: FileFormat = FileFormat {
    id: 61_913_269,
    puid: "wikidata/61913269",
    name: "Microsoft Project Export File, version 4",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    internal_signatures: &[],
    related_formats: &[],
};

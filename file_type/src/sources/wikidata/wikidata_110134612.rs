use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110134612: FileFormat = FileFormat {
    id: 110_134_612,
    puid: "wikidata/110134612",
    name: "Microsoft Publisher file format, version 2016",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};

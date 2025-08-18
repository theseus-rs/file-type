use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_15206305: FileType = FileType {
    file_format: &FileFormat {
        id: 15_206_305,
        source_type: SourceType::Wikidata,
        name: "Q15206305",
        extensions: &[],
        media_types: &[
            "application/vnd.docker.container.image.v1+json",
            "application/vnd.docker.distribution.manifest.list.v2+json",
            "application/vnd.docker.distribution.manifest.v1+json",
            "application/vnd.docker.distribution.manifest.v2+json",
            "application/vnd.docker.image.rootfs.diff.tar.gzip",
            "application/vnd.docker.image.rootfs.foreign.diff.tar.gzip",
            "application/vnd.docker.plugin.v1+json",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
